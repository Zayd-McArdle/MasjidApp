namespace MasjidApp.API.Library.Shared.Responses;

public interface IResponse
{
    bool IsSuccessful { get; }
    string ErrorReason { get; }
}