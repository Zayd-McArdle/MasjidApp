using MySql.Data.MySqlClient;

namespace MasjidApp.API.Library.Shared.DataAccess;

public sealed class DataAccessFactory(string connectionString) : IDataAccessFactory
{
    public IDataAccess EstablishDbConnection()
    {
        return new DataAccess<MySqlConnection>(connectionString);
    }
}