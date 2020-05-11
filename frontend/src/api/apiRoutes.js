const ApiBaseUrl = process.env.VUE_APP_BASE_URL
const BasePrefix = '/graphql'

const GraphQL = ApiBaseUrl + BasePrefix + '/v1'

const ApiRoutes = {
  GraphQL: GraphQL,
  EventSource: ApiBaseUrl + '/events'
}

export default ApiRoutes
