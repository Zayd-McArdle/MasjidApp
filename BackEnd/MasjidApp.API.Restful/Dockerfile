FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS base
USER $APP_UID
WORKDIR /app
EXPOSE 8080
EXPOSE 8081

FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
ARG BUILD_CONFIGURATION=Release
WORKDIR /src
COPY ["MasjidApp.API.Restful/MasjidApp.API.Restful.csproj", "MasjidApp.API.Restful/"]
RUN dotnet restore "MasjidApp.API.Restful/MasjidApp.API.Restful.csproj"
COPY . .
WORKDIR "/src/MasjidApp.API.Restful"
RUN dotnet build "MasjidApp.API.Restful.csproj" -c $BUILD_CONFIGURATION -o /app/build

FROM build AS publish
ARG BUILD_CONFIGURATION=Release
RUN dotnet publish "MasjidApp.API.Restful.csproj" -c $BUILD_CONFIGURATION -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "MasjidApp.API.Restful.dll"]
