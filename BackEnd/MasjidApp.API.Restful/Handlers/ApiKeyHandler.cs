namespace MasjidApp.API.Restful.Handlers;

public class ApiKeyHandler
{
    public static byte[] GetApiKey()
    {
        return "PleaseDoNotStoreThisHereChangeThisAndStoreItSecurely"u8.ToArray();
    }
}