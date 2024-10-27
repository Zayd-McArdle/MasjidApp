using MasjidApp.API.Library.Shared.Responses;

namespace MasjidApp.API.Library.Features.Authentication.Registration;

public sealed record RegistrationResponse : IResponse
{
    public bool IsSuccessful { get; init; }
    public string ErrorReason { get; init; }

    public static RegistrationResponse UserSuccessfullyRegistered()
    {
        return new RegistrationResponse()
        {
            IsSuccessful = true,
        };
    }

    public static RegistrationResponse UserAlreadyRegistered()
    {
        return new RegistrationResponse()
        {
            IsSuccessful = false,
            ErrorReason = "User already registered"
        };
    }

    public static RegistrationResponse FailedToRegister(string errorReason)
    {
        return new RegistrationResponse()
        {
            IsSuccessful = false,
            ErrorReason = errorReason
        };
    }

    
}