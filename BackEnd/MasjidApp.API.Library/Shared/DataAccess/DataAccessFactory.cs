using MySql.Data.MySqlClient;

namespace MasjidApp.API.Library.Shared.DataAccess;

public static class DataAccessFactory
{
    public static IDataAccess EstablishDbConnnection(string connectionString)
    {
        return new DataAccess<MySqlConnection>(connectionString);
    }
}