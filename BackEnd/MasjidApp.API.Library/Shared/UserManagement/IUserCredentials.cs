using System;

namespace MasjidApp.API.Library.Features.Authentication;

public interface IUserCredentials
{
    string Username { get; set; }
    string Password { get; set; }
}
