using System;
using System.ComponentModel.DataAnnotations;
using MasjidApp.API.Library.Shared.UserManagement;

namespace MasjidApp.API.Library.Features.Authentication.Registration;

public sealed class UserRegistrationRequest : IUserProfile, IUserCredentials
{
    [Required]
    public string FirstName { get; init; }
    
    [Required]
    public string LastName { get; init; }
    
    [EmailAddress]
    [Required]
    public string Email { get; init; }

    [Required]
    public string Role { get; set; }
    
    [Required]
    public string Username { get; set; }
    
    [Required]
    public string Password { get; set; }
}
