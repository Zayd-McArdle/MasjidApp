using MasjidApp.API.Library.Features.Authentication;
using MasjidApp.API.Library.Features.Authentication.Login;
using MasjidApp.API.Library.Features.Authentication.Registration;
using MasjidApp.API.Library.Features.Authentication.ResetPassword;
using MasjidApp.API.Library.Shared.UserManagement;
using MasjidApp.API.Restful.Controllers;
using MasjidApp.API.Restful.Handlers;
using Microsoft.AspNetCore.Mvc;
using Moq;

namespace MasjidApp.API.Testing.Unit.Features.Authentication;

public sealed class AuthenticationControllerTest 
{
    private readonly Mock<IUserRepository> _mockUserRepository;
    private readonly Mock<ITokenGenerator> _mockTokenGenerator;
    private readonly AuthenticationController _controller;
    private async Task TestInvalidModelState<T>(Func<T, Task<IActionResult>> performHttpRequest)
    where T : new()
    {
        // Given
        T request = new();

        // When
        _controller.ModelState.AddModelError("Session", "Required");
        IActionResult expected = _controller.BadRequest();
        IActionResult actual = await performHttpRequest(request);

        // Then
        Assert.Equivalent(expected, actual);
    }
    public AuthenticationControllerTest()
    {
        _mockUserRepository = new();
        _mockTokenGenerator = new();
        _controller = new(_mockUserRepository.Object, _mockTokenGenerator.Object);
    }

    #region Login Tests

    [InlineData(true, 0)]
    [InlineData(false, 0)]
    [InlineData(false, 1)]
    [Theory]
    public async Task Login_Test(bool checkBadModelState, int userCount)
    {
        if (checkBadModelState)
        {
            TestInvalidModelState<LoginRequest>(_controller.Login);
            return;
        }
        // Given
        LoginRequest request = new() {
            Username = "DummyUserName",
            Password = "DummyPassword"
        };
        IActionResult expectedResult = _controller.Unauthorized("Invalid username or password");
        _mockUserRepository.Setup(repository => repository.GetUserCredentials(request)).ReturnsAsync(userCount);
        
        // When
        if (userCount > 0)
        {
            expectedResult = _controller.Ok("DummyToken");
            _mockTokenGenerator.Setup(tokenGenerator => tokenGenerator.GenerateToken(It.IsAny<string>())).Returns("DummyToken");
        }
        IActionResult actualResult = await _controller.Login(request);
        // Then
        Assert.Equivalent(expectedResult, actualResult);
        
    }

    #endregion

    #region User Registration Tests
    
    [InlineData(true, null)]
    [InlineData(false, RegistrationResponse.UserSuccessfullyRegistered)]
    [InlineData(false, RegistrationResponse.UserAlreadyRegistered)]
    [Theory]
    public async Task RegisterUser_Test(bool checkBadModelState, RegistrationResponse mockResponse)
    {
        if (checkBadModelState)
        {
            TestInvalidModelState<UserRegistrationRequest>(_controller.RegisterUser);
            return;
        }
        // Given
        UserRegistrationRequest request = new() {
            FirstName = "first name",
            LastName = "last name",
            Email = "email@email.com",
            Role = nameof(UserRole.Normal),
            Username = "DummyUserName",
            Password = "DummyPassword"
        };
        _mockUserRepository.Setup(repository => repository.RegisterUser(It.IsAny<UserAccount>())).ReturnsAsync(mockResponse);
        
        // When
        IActionResult expected = _controller.Conflict($"Unable to register user {request.Username}");
        if (mockResponse == RegistrationResponse.UserSuccessfullyRegistered)
        {
            expected = _controller.Ok();
        }
        IActionResult actual = await _controller.RegisterUser(request);
        // Then
        Assert.Equivalent(expected, actual);
    }
    // [Fact]
    // public async Task RegisterUser_InternalServerError()
    // {
    //     // Given
    //     UserRegistrationRequest request = new() {
    //         FirstName = "first name",
    //         LastName = "last name",
    //         Email = "email@email.com",
    //         Role = nameof(UserRole.Normal),
    //         Username = "DummyUserName",
    //         Password = "DummyPassword"
    //     };
    //     RegistrationResponse mockResponse = RegistrationResponse.FailedToRegister;
    //     _mockUserRepository.Setup(repository => repository.RegisterUser(It.IsAny<UserAccount>())).ReturnsAsync(mockResponse);
    //     
    //     // When
    //     IActionResult expected = _controller.StatusCode(500, mockResponse.ErrorReason);
    //     IActionResult actual = await _controller.RegisterUser(request);
    //     // Then
    //     Assert.Equivalent(expected, actual);
    // }
    //
    #endregion

    #region Reset User Password Tests

    [InlineData(true, null)]
    [InlineData(false, ResetPasswordResponse.SuccessfullyResetUserPassword)]
    [InlineData(false, ResetPasswordResponse.UserDoesNotExist)]
    [Theory]
    public async Task ResetPassword_Test(bool checkBadModelState, ResetPasswordResponse mockResponse)
    {
        if (checkBadModelState)
        {
            TestInvalidModelState<ResetUserPasswordRequest>(_controller.ResetPassword);
            return;
        }
        // Given
        ResetUserPasswordRequest request = new() {
            Username = "DummyUserName",
            ReplacementPassword = "DummyPassword"
        };
        _mockUserRepository.Setup(repository => repository.ResetUserPassword(It.IsAny<string>(), It.IsAny<string>())).ReturnsAsync(mockResponse);
        
        // When
        IActionResult expected = mockResponse == ResetPasswordResponse.SuccessfullyResetUserPassword ? _controller.Ok() : _controller.NotFound();
        IActionResult actual = await _controller.ResetPassword(request);
        // Then
        Assert.Equivalent(expected, actual);
    }
    
    // [Fact]
    // public async Task ResetPassword_InternalServerError()
    // {
    //     // Given
    //     ResetUserPasswordRequest request = new() {
    //         Username = "DummyUserName",
    //         ReplacementPassword = "DummyPassword"
    //     };
    //     ResetPasswordResponse mockResponse = ResetPasswordResponse.FailedToResetUserPassword("Internal server error");
    //     _mockUserRepository.Setup(repository => repository.ResetUserPassword(It.IsAny<string>(), It.IsAny<string>())).ReturnsAsync(mockResponse);
    //     
    //     // When
    //     IActionResult expected = _controller.StatusCode(500, mockResponse.ErrorReason);
    //     IActionResult actual = await _controller.ResetPassword(request);
    //     // Then
    //     Assert.Equivalent(expected, actual);
    // }
    #endregion
}

