using System;
using System.Runtime;
using MasjidApp.API.Library.Features.Announcements;
using MasjidApp.API.Restful.Controllers;
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
        IEnumerable<Announcement>[] mockAnnouncements = [
            null, 
            [
                new Announcement {
                    Title = "Announcement1",
                    Description = "Description 1"
                },
                new Announcement {
                    Title = "Announcement2",
                    Description = "Description 2"
                }
             ]
        ];
        foreach (IEnumerable<Announcement> mockAnnouncement in mockAnnouncements)
        {
            _mockRepository.Setup(repository => repository.GetAnnouncements()).ReturnsAsync(mockAnnouncement);
            IEnumerable<Announcement> result = await _controller.GetAnnouncements();
            Assert.Equal(mockAnnouncement, result);
        }
    }

}
