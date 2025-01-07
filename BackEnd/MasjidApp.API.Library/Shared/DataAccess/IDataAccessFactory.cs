namespace MasjidApp.API.Library.Shared.DataAccess;

public interface IDataAccessFactory
{
    IDataAccess EstablishDbConnection();
}