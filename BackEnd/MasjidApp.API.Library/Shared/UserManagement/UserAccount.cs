using System;
using MasjidApp.API.Library.Features.Authentication;
using MasjidApp.API.Library.Features.Authentication.Security;

namespace MasjidApp.API.Library.Shared.UserManagement;

public sealed class UserAccount : IUserProfile, IUserCredentials
{
    public string FirstName { get; init; }
    public string LastName { get; init; }
    public string Email { get; init; }
    public string Role { get; set; }
    public string Username { get; set; }
    public string Password { get; set; }
}
