using MasjidApp.API.Library.Shared.Responses;

namespace MasjidApp.API.Library.Features.Authentication.ResetPassword;

public enum ResetPasswordResponse
{
    SuccessfullyResetUserPassword,
    UserDoesNotExist,
    FailedToResetUserPassword
}