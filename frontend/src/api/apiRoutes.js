const ApiBaseUrl = process.env.VUE_APP_BASE_URL

const ApiRoutes = {
  GraphQL: ApiBaseUrl + '/graphql',
  EventSource: ApiBaseUrl + '/notifications'
}

export default ApiRoutes
