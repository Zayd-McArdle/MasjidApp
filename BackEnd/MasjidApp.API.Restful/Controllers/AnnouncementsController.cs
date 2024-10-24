using MasjidApp.API.Library.Features.Announcements;
using MasjidApp.API.Restful.Handlers;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;

namespace MasjidApp.API.Restful.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class AnnouncementsController(IAnnouncementsRepository announcementsRepository) : ControllerBase
    {
        [HttpGet]
        public async Task<IEnumerable<AnnouncementDto>> GetAnnouncements() 
        {
            IEnumerable<AnnouncementDto> announcements = await announcementsRepository.GetAnnouncements();
            return announcements;
        }

        [Authorize]
        [HttpPost]
        public async Task<IActionResult> PostAnnouncement([FromBody] PostAnnouncementRequest request) 
        {
            if (!ModelState.IsValid)
            {
                return BadRequest();
            }

            return await DbExceptionHandler.HandleException(this, async () =>
            {
                AnnouncementDto announcement = new()
                {
                    Title = request.Title,
                    Description = request.Description,
                    Image = request.Image
                };
                AnnouncementsResponse response = await announcementsRepository.PostAnnouncement(announcement);
                if (response != AnnouncementsResponse.SuccessfullyPostedAnnouncement)
                {
                    // Return service unavailable 
                    return StatusCode(503, "There was an issue posting the announcement");
                }
                return Ok(); 
            });
        }

        [Authorize]
        [HttpPatch]
        public async Task<IActionResult> EditAnnouncement([FromBody] EditAnnouncementRequest request)
        {
            if (!ModelState.IsValid)
            {
                return BadRequest();
            }

            return await DbExceptionHandler.HandleException(this, async () =>
            {
                AnnouncementDto announcement = new()
                {
                    Title = request.Title,
                    Description = request.Description,
                    Image = request.Image
                };
                AnnouncementsResponse response = await announcementsRepository.EditAnnouncement(announcement);
                return response switch
                {
                    AnnouncementsResponse.AnnouncementNotFound => NotFound("The announcement requested does not exist"),
                    AnnouncementsResponse.NewAnnouncementMatchesOldAnnouncement => Conflict(
                        "The edited announcement matches the original"),
                    AnnouncementsResponse.FailedToEditAnnouncement =>
                        // Return service unavailable 
                        StatusCode(503, "There was an issue editing the announcement"),
                    _ => Ok()
                };
            });
        }
    }
}
