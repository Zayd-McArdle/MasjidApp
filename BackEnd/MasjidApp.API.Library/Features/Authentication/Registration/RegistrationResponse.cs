using MasjidApp.API.Library.Shared.Responses;

namespace MasjidApp.API.Library.Features.Authentication.Registration;

public enum RegistrationResponse
{
    UserSuccessfullyRegistered,
    UserAlreadyRegistered,
    FailedToRegister
}