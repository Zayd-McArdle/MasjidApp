using Microsoft.AspNetCore.Mvc;

namespace MasjidApp.API.Testing.Shared.Api;

internal static class ControllerTester
{
    internal static async Task TestInvalidModelState<T>(ControllerBase controller, Func<T, Task<IActionResult>> performHttpRequest)
        where T : new()
    {
        // Given
        T request = new();

        // When
        controller.ModelState.AddModelError("Session", "Required");
        IActionResult expected = controller.BadRequest();
        IActionResult actual = await performHttpRequest(request);

        // Then
        Assert.Equivalent(expected, actual);
    }
}