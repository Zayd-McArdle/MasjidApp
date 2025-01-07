using MasjidApp.API.Library.Features.PrayerTimes;
using MasjidApp.API.Restful.Controllers;
using Microsoft.AspNetCore.Mvc;
using Moq;
using MySql.Data.MySqlClient;

namespace MasjidApp.API.Testing.Unit.Features.PrayerTimes;

public class PrayerTimesControllerTests
{
    private readonly PrayerTimesController _controller;
    private readonly Mock<IPrayerTimesRepository> _prayerTimesRepositoryMock;

    public PrayerTimesControllerTests()
    {
        _prayerTimesRepositoryMock = new Mock<IPrayerTimesRepository>();
        _controller = new PrayerTimesController(_prayerTimesRepositoryMock.Object);
    }

    [InlineData(null)]
    [InlineData(new byte[] { 0x01, 0x02, 0x03 })]
    [Theory]
    public async Task GetPrayerTimesFile_Test(byte[] prayerTimesBytes)
    {
        // Arrange
        _prayerTimesRepositoryMock
            .Setup(repo => repo.GetPrayerTimes())
            .ReturnsAsync(prayerTimesBytes);

        // Act
        IActionResult result = await _controller.GetPrayerTimesFile();

        // Assert
        if (prayerTimesBytes != null)
        {
            FileStreamResult fileResult = Assert.IsType<FileStreamResult>(result);
            Assert.Equal("text/csv", fileResult.ContentType);
            Assert.Equal("prayerTimesFile.csv", fileResult.FileDownloadName);
        }
        else
        {
            Assert.Equivalent(result, _controller.NotFound());
        }

        // Check that the repository was called exactly once
        _prayerTimesRepositoryMock.Verify(repo => repo.GetPrayerTimes(), Times.Once);
    }

    [InlineData(null, false)]
    [InlineData(new byte[] { 0x01, 0x02, 0x03 }, false)]
    [InlineData(new byte[] { }, true)]
    [Theory]
    public async Task UpdatePrayerTimesFile_Test(byte[] fileData, bool isError)
    {
        PrayerTimesFileRequest request = null;
        if (fileData != null)
        {
            request = new()
            {
                FileData = fileData
            };
        }

        if (isError)
        {
            _prayerTimesRepositoryMock
                .Setup(repo => repo.UpdatePrayerTimes(It.IsAny<byte[]>()))
                .ReturnsAsync(PrayerTimesFileResponse.FailedToUpdatePrayerTimesFile);
        }

        // Act
        IActionResult result = await _controller.UpdatePrayerTimesFile(request);

        // Assert
        if (request == null)
        {
            BadRequestObjectResult badRequestResult = Assert.IsType<BadRequestObjectResult>(result);
            Assert.Equal("Invalid file uploaded", badRequestResult.Value);
        }
        else if (isError)
        {
            // Act
            IActionResult expected = _controller.StatusCode(500);
            // Assert
            Assert.Equivalent(expected, result);
        }
        else
        {
            Assert.IsType<OkResult>(result);
        }
    }
}