using MasjidApp.API.Library.Features.Authentication;
using MasjidApp.API.Library.Features.Authentication.Login;
using MasjidApp.API.Library.Features.Authentication.Registration;
using MasjidApp.API.Library.Features.Authentication.ResetPassword;
using MasjidApp.API.Library.Features.Authentication.Security;
using MasjidApp.API.Library.Shared.UserManagement;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Mvc;

namespace MasjidApp.API.Restful.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class AuthenticationController(IUserRepository userRepository) : ControllerBase
    {
        [HttpPost]
        public async Task<IActionResult> Login([FromBody] LoginRequest request) 
        {
            if (request.Username == null || request.Password == null)
            {
                return BadRequest("Invalid request");
            }
            int loginCount = await userRepository.GetUserCredentials(request);
            if (loginCount == 0) 
            {
                return Unauthorized("Invalid username or password");
            }
            return Ok();
        }

        [HttpPost]
        public async Task<IActionResult> RegisterUser([FromBody] UserRegistrationRequest request)
        {
            if (!ModelState.IsValid)
            {
                return BadRequest();
            }
            RegistrationResponse response = await userRepository.RegisterUser(new UserAccount {
                FirstName = request.FirstName,
                LastName = request.LastName,
                Email = request.Email,
                Role = request.Role,
                Username = request.Username,
                Password = request.Password
            });
            if (response.IsSuccessful)
            {
                return Ok();
            }
            else if (response == RegistrationResponse.UserAlreadyRegistered())
            {
                return Conflict(response.ErrorReason);
            }
            return StatusCode(500, response.ErrorReason);
        }

        [HttpPatch]
        public async Task<IActionResult> ResetPassword([FromBody] ResetUserPasswordRequest request)
        {
            if (!ModelState.IsValid)
            {
                return BadRequest();
            }
            ResetPasswordResponse response = await userRepository.ResetUserPassword(request.Username, request.ReplacementPassword);
            if (response.IsSuccessful)
            {
                return Ok();
            }
            else if (response == ResetPasswordResponse.UserDoesNotExist())
            {
                return NotFound(response.ErrorReason);
            }
            return StatusCode(500, response.ErrorReason);
        }
    }
}
