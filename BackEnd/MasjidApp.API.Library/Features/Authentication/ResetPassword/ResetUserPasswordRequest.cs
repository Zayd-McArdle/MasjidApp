using System;
using System.ComponentModel.DataAnnotations;

namespace MasjidApp.API.Library.Features.Authentication.ResetPassword;

public sealed class ResetUserPasswordRequest
{
    [Required]
    public string Username { get; init; }

    [Required]
    public string ReplacementPassword { get; init; }

}
