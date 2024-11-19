namespace MasjidApp.API.Restful.Handlers;

public interface ITokenGenerator
{
    string GenerateToken(string username);
}