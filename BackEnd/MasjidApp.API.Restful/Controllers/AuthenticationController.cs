using MasjidApp.API.Library.Features.Authentication;
using MasjidApp.API.Library.Features.Authentication.Login;
using MasjidApp.API.Library.Features.Authentication.Registration;
using MasjidApp.API.Library.Features.Authentication.ResetPassword;
using MasjidApp.API.Library.Shared.UserManagement;
using MasjidApp.API.Restful.Handlers;
using Microsoft.AspNetCore.Mvc;

namespace MasjidApp.API.Restful.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class AuthenticationController(IUserRepository userRepository, ITokenGenerator tokenGenerator) : ControllerBase
    {
        [HttpPost("login")]
        public async Task<IActionResult> Login([FromBody] LoginRequest request) 
        {
            if (!ModelState.IsValid)
            {
                return BadRequest();
            }

            return await DbExceptionHandler.HandleException(this, async () =>
            {
                LoginDto loginCount = await userRepository.GetUserCredentials(request);
                if (loginCount == null) 
                {
                    return Unauthorized("Invalid username or password");
                }
                return Ok(tokenGenerator.GenerateToken(request.Username));
            });
        }

        [HttpPost("register-user")]
        public async Task<IActionResult> RegisterUser([FromBody] UserRegistrationRequest request)
        {
            if (!ModelState.IsValid)
            {
                return BadRequest();
            }
            return await DbExceptionHandler.HandleException(this, async () =>
            {
                RegistrationResponse response = await userRepository.RegisterUser(new UserAccount
                {
                    FirstName = request.FirstName,
                    LastName = request.LastName,
                    Email = request.Email,
                    Role = request.Role,
                    Username = request.Username,
                    Password = request.Password
                });
                if (response != RegistrationResponse.UserSuccessfullyRegistered)
                {
                    if (response == RegistrationResponse.UserAlreadyRegistered)
                    {
                        return Conflict($"Unable to register user {request.Username}");
                    }
                }

                return Ok();
                
            });
        }

        [HttpPatch("reset-password")]
        public async Task<IActionResult> ResetPassword([FromBody] ResetUserPasswordRequest request)
        {
            if (!ModelState.IsValid)
            {
                return BadRequest();
            }

            return await DbExceptionHandler.HandleException(this, async () =>
            {
                ResetPasswordResponse response = await userRepository.ResetUserPassword(request.Username, request.ReplacementPassword);
                if (response != ResetPasswordResponse.SuccessfullyResetUserPassword)
                {
                    if (response == ResetPasswordResponse.UserDoesNotExist)
                    {
                        return NotFound();
                    }
                }

                return Ok();
            });
            
        }
    }
}
