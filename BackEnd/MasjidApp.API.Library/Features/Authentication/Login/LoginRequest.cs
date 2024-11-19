using System;
using System.ComponentModel.DataAnnotations;

namespace MasjidApp.API.Library.Features.Authentication.Login;

public sealed class LoginRequest : IUserCredentials
{
    [MinLength(6)]
    [Required]
    public string Username { get; set; }
    [Required]
    public string Password { get; set; }
}
