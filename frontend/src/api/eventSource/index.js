/* exposes only one function/entry point, which is the initial setup of the connection. Inside this function, eventsource/handlers are registered.
 * handlers/index.js contains the manifest of all the handlers in use. */

import ApiRoutes from '@/api/apiRoutes'
import Handlers from './handlers'

export default function configureEventSources () {
  const eventSource = new EventSource(ApiRoutes.EventSource)
  for (const handler of Handlers) {
    eventSource.addEventListener(handler.eventType, event => {
      handler.handle(event)
    })
  }
}
