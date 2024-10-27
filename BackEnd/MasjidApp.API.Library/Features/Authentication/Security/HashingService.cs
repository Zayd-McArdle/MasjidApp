namespace MasjidApp.API.Library.Features.Authentication.Security;

public static class HashingService
{
    public static string HashCredential(string credential)
    {
        return BCrypt.Net.BCrypt.HashPassword(credential);
    }
    public static void HashCredentials(IUserCredentials credentials)
    {
        credentials.Username = BCrypt.Net.BCrypt.HashPassword(credentials.Username);
        credentials.Password = BCrypt.Net.BCrypt.HashPassword(credentials.Password);
    }
}