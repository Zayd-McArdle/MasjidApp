using MySql.Data.MySqlClient;

namespace MasjidApp.API.Library.Shared.DataAccess;

public static class DataAccessFactory
{
    public static IDataAccess CreateDataAccess(string connectionString)
    {
        return new DataAccess<MySqlConnection>(connectionString);
    }
}