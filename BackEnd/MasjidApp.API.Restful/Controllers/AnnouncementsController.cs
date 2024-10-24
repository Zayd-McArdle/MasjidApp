using MasjidApp.API.Library.Features.Announcements;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Http;
using Microsoft.AspNetCore.Mvc;

namespace MasjidApp.API.Restful.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class AnnouncementsController : ControllerBase
    {
        private readonly IAnnouncementsRepository _announcementsRepository;
        public AnnouncementsController(IAnnouncementsRepository announcementsRepository)
        {
            _announcementsRepository = announcementsRepository;
        }

        [HttpGet]
        public async Task<IEnumerable<Announcement>> GetAnnouncements() 
        {
            IEnumerable<Announcement> announcements = await _announcementsRepository.GetAnnouncements();
            return announcements;
        }

        [Authorize]
        [HttpPost]
        public async Task<IActionResult> PostAnnouncement(Announcement announcement) 
        {
            try
            {
                await _announcementsRepository.PostAnnouncement(announcement);
                return Ok();
            }
            catch (Exception)
            {
                
            }
            
        }
    }
}
