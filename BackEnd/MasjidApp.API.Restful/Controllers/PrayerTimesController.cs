using MasjidApp.API.Library.Features.PrayerTimes;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using MySql.Data.MySqlClient;

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

            try
            {
                await prayerTimesRepository.UpdatePrayerTimes(request.FileData);
            }
            catch (MySqlException ex)
            {
#if DEBUG
                string errorMessage = $"Database error occurred {ex.Message}";
#else
                string errorMessage = $"Unable to update prayer times file due to a database error.";
#endif
                return StatusCode(500, errorMessage);
            }
            catch (Exception ex)
            {
                return StatusCode(500, ex);
            }

            return Ok();
        }
    }
}