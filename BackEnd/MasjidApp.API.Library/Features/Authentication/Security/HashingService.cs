namespace MasjidApp.API.Library.Features.Authentication.Security;

public static class HashingService
{
    public static string HashCredential(string credential)
    {
        return BCrypt.Net.BCrypt.HashPassword(credential);
    }

    public static bool HashVerified(string credential, string hash)
    {
        return BCrypt.Net.BCrypt.Verify(credential, hash);
    }
}