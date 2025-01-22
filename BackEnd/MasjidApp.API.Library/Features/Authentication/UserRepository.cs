using MasjidApp.API.Library.Features.Authentication.Registration;
using MasjidApp.API.Library.Features.Authentication.ResetPassword;
using MasjidApp.API.Library.Features.Authentication.Security;
using MasjidApp.API.Library.Shared.DataAccess;
using MasjidApp.API.Library.Shared.UserManagement;

namespace MasjidApp.API.Library.Features.Authentication;

public sealed class UserRepository(IDataAccessFactory dataAccessFactory) : IUserRepository
{
    private static async Task<bool> UserExistsInDatabase(IDataAccess connection, string username)
    {
        int userCount = await connection.ReadRecordCountFromDatabaseAsync("get_username", new {p_username = username});
        return userCount > 0;
    }
    public async Task<int> GetUserCredentials(IUserCredentials credentials)
    {
        using IDataAccess connection = dataAccessFactory.EstablishDbConnection();
        int userCount = await connection.ReadRecordCountFromDatabaseAsync<dynamic>("get_user_credentials", new { p_username = credentials.Username, p_password = credentials.Password});
        return userCount;
    }

    public async Task<RegistrationResponse> RegisterUser(UserAccount newUser)
    {
        using IDataAccess connection = dataAccessFactory.EstablishDbConnection();
        HashingService.HashCredentials(newUser);
        bool userExists = await UserExistsInDatabase(connection, newUser.Username);
        if (userExists)
        {
            return RegistrationResponse.UserAlreadyRegistered;
        }

        try
        {
            await connection.WriteToDatabaseAsync("register_user", new { 
                p_first_name = newUser.FirstName, 
                p_last_name = newUser.LastName, 
                p_role = newUser.Role, 
                p_email = newUser.Email,
                p_username = newUser.Username,
                p_password = newUser.Password
                
            });
            
            userExists = await UserExistsInDatabase(connection, newUser.Username);
            
            return userExists ? RegistrationResponse.UserSuccessfullyRegistered : RegistrationResponse.FailedToRegister;
        }
        catch (Exception ex)
        {
            return RegistrationResponse.FailedToRegister;
        }
    }

    public async Task<ResetPasswordResponse> ResetUserPassword(string username, string newPassword)
    {
        using IDataAccess connection = dataAccessFactory.EstablishDbConnection();
        username = HashingService.HashCredential(username);
        bool userExists = await UserExistsInDatabase(connection, username);
        if (!userExists)
        {
            return ResetPasswordResponse.UserDoesNotExist;
        }
        HashingService.HashCredential(newPassword);
        await connection.WriteToDatabaseAsync("reset_user_password", new { p_username = username, p_password = newPassword });
        return ResetPasswordResponse.SuccessfullyResetUserPassword;
    }

    public async Task<bool> UserRegistered(string username)
    {
        using IDataAccess connection = dataAccessFactory.EstablishDbConnection();
        username = HashingService.HashCredential(username);
        return await UserExistsInDatabase(connection, username);
    }
}
