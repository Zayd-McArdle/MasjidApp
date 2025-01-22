namespace MasjidApp.API.Library.Features.Authentication.Login;
public class LoginDto : IUserCredentials
{
    public int Id { get; set; }
    public string Username { get; set; }
    public string Password { get; set; }
}