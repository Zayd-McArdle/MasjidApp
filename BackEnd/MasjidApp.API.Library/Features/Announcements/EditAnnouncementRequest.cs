using System.ComponentModel.DataAnnotations;

namespace MasjidApp.API.Library.Features.Announcements;

public sealed class EditAnnouncementRequest
{
    [Required]
    public int AnnouncementId { get; init; }
    [Required]
    public string Title { get; init; }
    [Required]
    public string Description { get; init; }
    public byte[] Image { get; init; }
}