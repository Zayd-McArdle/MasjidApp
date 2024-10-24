using System;

namespace MasjidApp.API.Library.Features.Announcements;

public sealed record AnnouncementDto
{
    public int Id { get; init; }
    public required string Title { get; init; }
    public required string Description { get; init; }
    public byte[] Image { get; init; }
    public DateTime LastUpdated { get; init; }
}
