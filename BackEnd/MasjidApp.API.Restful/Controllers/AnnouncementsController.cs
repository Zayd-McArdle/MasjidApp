using MasjidApp.API.Library.Features.Announcements;
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
        public async Task<IEnumerable<Announcement>> GetAnnouncements() 
        {
            IEnumerable<Announcement> announcements = await announcementsRepository.GetAnnouncements();
            return announcements;
        }

        [Authorize]
        [HttpPost]
        public async Task<IActionResult> PostAnnouncement(Announcement announcement) 
        {
            try
            {
                await announcementsRepository.PostAnnouncement(announcement);
                return Ok();
            }
            catch (Exception)
            {
                return StatusCode(500);
            }
            
        }
    }
}
