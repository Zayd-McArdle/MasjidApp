using MasjidApp.API.Library.Shared.Responses;

namespace MasjidApp.API.Library.Features.Authentication.ResetPassword;

public sealed record ResetPasswordResponse : IResponse
{
    public bool IsSuccessful { get; init; }
    public string ErrorReason { get; init; }

    public static ResetPasswordResponse SuccessfullyResetUserPassword()
    {
        return new ResetPasswordResponse { IsSuccessful = true };
    }

    public static ResetPasswordResponse UserDoesNotExist()
    {
        return new ResetPasswordResponse { ErrorReason = "User does not exist" };
    }
    public static ResetPasswordResponse FailedToResetUserPassword(string errorReason)
    {
        return new ResetPasswordResponse { ErrorReason = errorReason };
    }
}