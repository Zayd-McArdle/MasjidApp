using System;
using MasjidApp.API.Library.Features.Authentication;

namespace MasjidApp.API.Library.Shared.UserManagement;

public interface IUserProfile
{
    string FirstName { get; init; }
    string LastName { get; init; }
    string Email { get; init; }
    string Role { get; set; }
}
