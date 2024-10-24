using MasjidApp.API.Library.Shared.DataAccess;

namespace MasjidApp.API.Library.Features.Announcements;

public sealed class AnnouncementsRepository(IDataAccessFactory dataAccessFactory) : IAnnouncementsRepository
{
    public async Task<IEnumerable<AnnouncementDto>> GetAnnouncements()
    {
        using IDataAccess dataAccess = dataAccessFactory.EstablishDbConnection();
        IEnumerable<AnnouncementDto> announcements = await dataAccess.ReadRecordsFromDatabaseAsync<AnnouncementDto>("get_announcements");
        return announcements;
    }

    public async Task<AnnouncementsResponse> PostAnnouncement(AnnouncementDto announcement)
    {
        using IDataAccess dataAccess = dataAccessFactory.EstablishDbConnection();
        int id = await dataAccess.WriteToDatabaseAsyncWithVerification("post_announcement", new
        {
            p_title = announcement.Title, 
            p_description = announcement.Description, 
            p_image = announcement.Image, 
        });
        if (id == 0)
        {
            return AnnouncementsResponse.FailedToPostAnnouncement;
        }
        return AnnouncementsResponse.SuccessfullyPostedAnnouncement;
    }

    public async Task<AnnouncementsResponse> EditAnnouncement(AnnouncementDto announcement)
    {
        using IDataAccess dataAccess = dataAccessFactory.EstablishDbConnection();
        AnnouncementDto oldAnnouncementDto = await dataAccess.ReadFirstRecordFromDatabaseWithParametersAsync<AnnouncementDto, dynamic>("get_announcement", new { p_id = announcement.Id });
        if (oldAnnouncementDto == null)
        {
            return AnnouncementsResponse.AnnouncementNotFound;
        }
        if (oldAnnouncementDto == announcement)
        {
            return AnnouncementsResponse.NewAnnouncementMatchesOldAnnouncement;
        }
        await dataAccess.WriteToDatabaseAsync("edit_announcement", new {
            p_id = announcement.Id, 
            p_title = announcement.Title, 
            p_description = announcement.Description, 
            p_image = announcement.Image, 
        });
        AnnouncementDto editedAnnouncementDto = await dataAccess.ReadFirstRecordFromDatabaseWithParametersAsync<AnnouncementDto, dynamic>("get_announcement", new { p_id = announcement.Id });
        if (oldAnnouncementDto == editedAnnouncementDto)
        {
            return AnnouncementsResponse.FailedToEditAnnouncement;
        }

        return AnnouncementsResponse.SuccessfullyEditedAnnouncement;
    }
}
