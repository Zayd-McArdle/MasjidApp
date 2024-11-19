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

    [Fact]
    public async Task Login_InvalidModelState()
    {
        // Given
        LoginRequest request = new();

        // When
        IActionResult expected = _controller.BadRequest("Invalid request");
        IActionResult actual = await _controller.Login(request);

        // Then
        Assert.Equivalent(expected, actual);
    }
    [Fact]
    public async Task Login_UserNotFound()
    {
        // Given
        LoginRequest request = new() {
            Username = "DummyUserName",
            Password = "DummyPassword"
        };
        _mockUserRepository.Setup(repository => repository.GetUserCredentials(request)).ReturnsAsync(0);
        
        // When
        IActionResult expected = _controller.Unauthorized("Invalid username or password");
        IActionResult actual = await _controller.Login(request);
        // Then
        Assert.Equivalent(expected, actual);
    }

    [Fact]
    public async Task Login_UserFound()
    {
        // Given
        LoginRequest request = new() {
            Username = "DummyUserName",
            Password = "DummyPassword"
        };
        _mockUserRepository.Setup(repository => repository.GetUserCredentials(request)).ReturnsAsync(1);
        _mockTokenGenerator.Setup(tokenGenerator => tokenGenerator.GenerateToken(It.IsAny<string>())).Returns("DummyToken");
        // When
        IActionResult expected = _controller.Ok("DummyToken");
        IActionResult actual = await _controller.Login(request);
        // Then
        Assert.Equivalent(expected, actual);
    }

    #endregion

    #region User Registration Tests

    [Fact]
    public async Task RegisterUser_InvalidModelState()
    {
        await TestInvalidModelState<UserRegistrationRequest>(_controller.RegisterUser);
    }

    [Fact]
    public async Task RegisterUser_UserAlreadyExists()
    {
        // Given
        UserRegistrationRequest request = new() {
            FirstName = "first name",
            LastName = "last name",
            Email = "email@email.com",
            Role = nameof(UserRole.Normal),
            Username = "DummyUserName",
            Password = "DummyPassword"
        };
        RegistrationResponse mockResponse = RegistrationResponse.UserAlreadyRegistered();
        _mockUserRepository.Setup(repository => repository.RegisterUser(It.IsAny<UserAccount>())).ReturnsAsync(mockResponse);
        
        // When
        IActionResult expected = _controller.Conflict(mockResponse.ErrorReason);
        IActionResult actual = await _controller.RegisterUser(request);
        // Then
        Assert.Equivalent(expected, actual);
    }

    [Fact]
    public async Task RegisterUser_RegistrationSuccessful()
    {
        // Given
        UserRegistrationRequest request = new() {
            FirstName = "first name",
            LastName = "last name",
            Email = "email@email.com",
            Role = nameof(UserRole.Normal),
            Username = "DummyUserName",
            Password = "DummyPassword"
        };
        RegistrationResponse mockResponse = RegistrationResponse.UserSuccessfullyRegistered();
        _mockUserRepository.Setup(repository => repository.RegisterUser(It.IsAny<UserAccount>())).ReturnsAsync(mockResponse);
        
        // When
        IActionResult expected = _controller.Ok();
        IActionResult actual = await _controller.RegisterUser(request);
        // Then
        Assert.Equivalent(expected, actual);
    }
    
    [Fact]
    public async Task RegisterUser_InternalServerError()
    {
        // Given
        UserRegistrationRequest request = new() {
            FirstName = "first name",
            LastName = "last name",
            Email = "email@email.com",
            Role = nameof(UserRole.Normal),
            Username = "DummyUserName",
            Password = "DummyPassword"
        };
        RegistrationResponse mockResponse = RegistrationResponse.FailedToRegister("Internal server error");
        _mockUserRepository.Setup(repository => repository.RegisterUser(It.IsAny<UserAccount>())).ReturnsAsync(mockResponse);
        
        // When
        IActionResult expected = _controller.StatusCode(500, mockResponse.ErrorReason);
        IActionResult actual = await _controller.RegisterUser(request);
        // Then
        Assert.Equivalent(expected, actual);
    }
    
    #endregion

    #region Reset User Password Tests

    [Fact]
    public async Task ResetPassword_InvalidModelState()
    {
        await TestInvalidModelState<ResetUserPasswordRequest>(_controller.ResetPassword);
    }

    [Fact]
    public async Task ResetPassword_UserDoesNotExist()
    {
        // Given
        ResetUserPasswordRequest request = new() {
            Username = "DummyUserName",
            ReplacementPassword = "DummyPassword"
        };
        ResetPasswordResponse mockResponse = ResetPasswordResponse.UserDoesNotExist();
        _mockUserRepository.Setup(repository => repository.ResetUserPassword(It.IsAny<string>(), It.IsAny<string>())).ReturnsAsync(mockResponse);
        
        // When
        IActionResult expected = _controller.NotFound(mockResponse.ErrorReason);
        IActionResult actual = await _controller.ResetPassword(request);
        // Then
        Assert.Equivalent(expected, actual);
    }

    [Fact]
    public async Task ResetPassword_Success()
    {
        // Given
        ResetUserPasswordRequest request = new() {
            Username = "DummyUserName",
            ReplacementPassword = "DummyPassword"
        };
        ResetPasswordResponse mockResponse = ResetPasswordResponse.SuccessfullyResetUserPassword();
        _mockUserRepository.Setup(repository => repository.ResetUserPassword(It.IsAny<string>(), It.IsAny<string>())).ReturnsAsync(mockResponse);
        
        // When
        IActionResult expected = _controller.Ok();
        IActionResult actual = await _controller.ResetPassword(request);
        
        // Then
        Assert.Equivalent(expected, actual);
    }
    
    [Fact]
    public async Task ResetPassword_InternalServerError()
    {
        // Given
        ResetUserPasswordRequest request = new() {
            Username = "DummyUserName",
            ReplacementPassword = "DummyPassword"
        };
        ResetPasswordResponse mockResponse = ResetPasswordResponse.FailedToResetUserPassword("Internal server error");
        _mockUserRepository.Setup(repository => repository.ResetUserPassword(It.IsAny<string>(), It.IsAny<string>())).ReturnsAsync(mockResponse);
        
        // When
        IActionResult expected = _controller.StatusCode(500, mockResponse.ErrorReason);
        IActionResult actual = await _controller.ResetPassword(request);
        // Then
        Assert.Equivalent(expected, actual);
    }
    #endregion
}

