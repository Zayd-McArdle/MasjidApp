using MasjidApp.API.Library.Features.Authentication;
using MasjidApp.API.Restful.Handlers;
using Microsoft.AspNetCore.Authentication.JwtBearer;
using Microsoft.IdentityModel.Tokens;
using MasjidApp.API.Library.Features.PrayerTimes;
using MasjidApp.API.Library.Shared.DataAccess;

WebApplicationBuilder builder = WebApplication.CreateBuilder(args);

// Add services to the container.
builder.Services.AddSingleton<ITokenGenerator, TokenGenerator>(provider =>
{
    IConfiguration configuration = provider.GetRequiredService<IConfiguration>();
    return new TokenGenerator(configuration);
});
builder.Services.AddSingleton<IUserRepository, UserRepository>(provider =>
{
    IConfiguration configuration = provider.GetRequiredService<IConfiguration>();
    return new UserRepository(new DataAccessFactory(configuration.GetConnectionString("AuthenticationConnection")));
});
builder.Services.AddSingleton<IPrayerTimesRepository, PrayerTimesRepository>(provider =>
{
    IConfiguration configuration = provider.GetRequiredService<IConfiguration>();
    return new PrayerTimesRepository(new DataAccessFactory(configuration.GetConnectionString("PrayerTimesConnection")));
});
builder.Services.AddControllers();
// Learn more about configuring Swagger/OpenAPI at https://aka.ms/aspnetcore/swashbuckle
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();
builder.Services.AddAuthorization();
builder.Services.AddAuthentication(JwtBearerDefaults.AuthenticationScheme).AddJwtBearer(x =>
{
    x.TokenValidationParameters = new TokenValidationParameters
    {
        IssuerSigningKey = new SymmetricSecurityKey(ApiKeyHandler.GetApiKey()),
        ValidIssuer = builder.Configuration["Jwt:Issuer"],
        ValidAudience = builder.Configuration["Jwt:Audience"], 
        ValidateIssuerSigningKey = true,
        ValidateLifetime = true,
        ValidateIssuer = true,
        ValidateAudience = true
    };
});
WebApplication app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseHttpsRedirection();

app.UseAuthentication();
app.UseAuthorization();

app.MapControllers();

app.Run();