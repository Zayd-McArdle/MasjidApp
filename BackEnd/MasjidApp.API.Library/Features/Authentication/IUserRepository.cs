using System;
using MasjidApp.API.Library.Features.Authentication.Login;
using MasjidApp.API.Library.Features.Authentication.Registration;
using MasjidApp.API.Library.Features.Authentication.ResetPassword;
using MasjidApp.API.Library.Shared.UserManagement;
namespace MasjidApp.API.Library.Features.Authentication;

public interface IUserRepository
{
    Task<int> GetUserCredentials(IUserCredentials userCredentials);
    Task<RegistrationResponse> RegisterUser(UserAccount newUser);
    Task<ResetPasswordResponse> ResetUserPassword(string username, string newPassword);
    Task<bool> UserRegistered(string username);
}
