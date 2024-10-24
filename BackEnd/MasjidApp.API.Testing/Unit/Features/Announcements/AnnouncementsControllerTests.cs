using MasjidApp.API.Library.Features.Announcements;
using MasjidApp.API.Restful.Controllers;
using MasjidApp.API.Testing.Shared.Api;
using Microsoft.AspNetCore.Mvc;
using Moq;

namespace MasjidApp.API.Testing.Unit.Features.Announcements;

public class AnnouncementsControllerTests
{
    private readonly AnnouncementsController _controller;
    private readonly Mock<IAnnouncementsRepository> _mockRepository;
    public AnnouncementsControllerTests()
    {
        _mockRepository = new();
        _controller = new(_mockRepository.Object);
    }

    [Fact]
    public async Task GetAnnouncements_Test()
    {
        IEnumerable<AnnouncementDto>[] mockAnnouncements = [
            null, 
            [
                new AnnouncementDto {
                    Title = "Announcement1",
                    Description = "Description 1"
                },
                new AnnouncementDto {
                    Title = "Announcement2",
                    Description = "Description 2"
                }
             ]
        ];
        foreach (IEnumerable<AnnouncementDto> mockAnnouncement in mockAnnouncements)
        {
            _mockRepository.Setup(repository => repository.GetAnnouncements()).ReturnsAsync(mockAnnouncement);
            IEnumerable<AnnouncementDto> result = await _controller.GetAnnouncements();
            Assert.Equal(mockAnnouncement, result);
        }
    }
    
    [InlineData(true, null)]
    [InlineData(false, AnnouncementsResponse.FailedToPostAnnouncement)]
    [InlineData(false, AnnouncementsResponse.SuccessfullyPostedAnnouncement)]
    [Theory]
    public async Task PostAnnouncement_Test(bool checkBadModelState, AnnouncementsResponse mockResponse)
    {
        if (checkBadModelState)
        {
            await ControllerTester.TestInvalidModelState<PostAnnouncementRequest>(_controller, _controller.PostAnnouncement);
            return;
        }

        IActionResult expectedResult = mockResponse == AnnouncementsResponse.SuccessfullyPostedAnnouncement? _controller.Ok() : _controller.StatusCode(503, "There was an issue posting the announcement");
        PostAnnouncementRequest request = new PostAnnouncementRequest()
        {
            Title = "Announcement title",
            Description = "description of announcement",
            Image = [0x1, 0x2, 0x3]
        };
        
        _mockRepository.Setup(repository => repository.PostAnnouncement(It.IsAny<AnnouncementDto>())).ReturnsAsync(mockResponse);
        IActionResult actualResult = await _controller.PostAnnouncement(request);
        Assert.Equivalent(expectedResult, actualResult);
    }

    [InlineData(true, false, null)]
    [InlineData(false, false, AnnouncementsResponse.AnnouncementNotFound)]
    [InlineData(false, false, AnnouncementsResponse.NewAnnouncementMatchesOldAnnouncement)]
    [InlineData(false, false, AnnouncementsResponse.FailedToEditAnnouncement)]
    [InlineData(false, false, AnnouncementsResponse.SuccessfullyEditedAnnouncement)]
    [Theory]
    public async Task EditAnnouncement_Test(bool checkBadModelState, bool throwsException, AnnouncementsResponse mockResponse)
    {
        if (checkBadModelState)
        {
            await ControllerTester.TestInvalidModelState<PostAnnouncementRequest>(_controller, _controller.PostAnnouncement);
            return;
        }

        IActionResult expectedResult = null;
        switch (mockResponse)
        {
            case AnnouncementsResponse.AnnouncementNotFound:
                expectedResult = _controller.NotFound("The announcement requested does not exist");
                break;
            case AnnouncementsResponse.NewAnnouncementMatchesOldAnnouncement:
                expectedResult = _controller.Conflict("The edited announcement matches the original");
                break;
            case AnnouncementsResponse.FailedToEditAnnouncement:
                expectedResult = _controller.StatusCode(503, "There was an issue editing the announcement");
                break;
            case AnnouncementsResponse.SuccessfullyEditedAnnouncement:
                expectedResult = _controller.Ok();
                break;
        }
        EditAnnouncementRequest request = new ()
        {
            AnnouncementId = 1,
            Title = "Edited Announcement title",
            Description = "description of edited announcement",
            Image = [0x1, 0x2, 0x3]
        };
        _mockRepository.Setup(repository => repository.EditAnnouncement(It.IsAny<AnnouncementDto>())).ReturnsAsync(mockResponse);
        IActionResult actualResult = await _controller.EditAnnouncement(request);
        Assert.Equivalent(expectedResult, actualResult);
    }
}
