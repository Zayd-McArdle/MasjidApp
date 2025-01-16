using MasjidApp.API.Library.Features.PrayerTimes;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;

namespace MasjidApp.API.Restful.Controllers
{
    [Route("api/[controller]")]
    [ApiController]
    public class PrayerTimesController(IPrayerTimesRepository prayerTimesRepository) : ControllerBase
    {
        [HttpGet]
        public async Task<IActionResult> GetPrayerTimesFile()
        {
            byte[] prayerTimesBytes = await prayerTimesRepository.GetPrayerTimes();
            if (prayerTimesBytes == null)
            {
                return NotFound();
            }
            return File(new MemoryStream(prayerTimesBytes), "text/csv", "prayerTimesFile.csv");
        }
        
        [Authorize]
        [HttpPut]
        public async Task<IActionResult> UpdatePrayerTimesFile([FromBody] PrayerTimesFileRequest? request)
        {
            if (request?.FileData == null)
            {
                return BadRequest("Invalid file uploaded");
            }

            PrayerTimesFileResponse response = await prayerTimesRepository.UpdatePrayerTimes(request.FileData);
            if (response == PrayerTimesFileResponse.FailedToUpdatePrayerTimesFile)
            {
                return StatusCode(500);
            }
            return Ok();
        }
    }
}