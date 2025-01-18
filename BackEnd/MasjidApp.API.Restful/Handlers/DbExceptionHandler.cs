using Microsoft.AspNetCore.Mvc;

namespace MasjidApp.API.Restful.Handlers;

public static class DbExceptionHandler
{
    public static async Task<IActionResult> HandleException(ControllerBase controller, Func<Task<IActionResult>> httpMethod)
    {
        try
        {
            return await httpMethod();
        }
        catch (Exception ex)
        {
            if (ex.Message.Contains("Unable to connect to any of the specified MySQL hosts"))
            {
                return controller.StatusCode(503, "Unable to retrieve resources at this given time. Please try again later.");
            }
            throw;
        }
    }
}