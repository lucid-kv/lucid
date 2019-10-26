import { LUCID_SERVER_URI } from './main'
import store from './store'

/** Lucid kv endpoint */
export const kv = `${LUCID_SERVER_URI}/api/kv`

/**
 * Call the Lucid API
 *
 * @param {string} key Targetted key
 * @param {string} method HTTP method
 * @param {string} body Request body
 * @param {object} headers Headers to add to the request
 * @returns {Promise<Response>} Request response
 * @throws {Error} Not logged in - GET, DELETE, HEAD with body - Missing PUT body - Request error
 */
export const apiCall = async (key, method = 'GET', body, headers = {}) => {
  // Check logged in
  if (!store.state.token) throw new Error('You must be logged in to request a key-value pair.')

  // Check if trying to do a request with a body when not allowed to
  const noBodyMethods = ['GET', 'DELETE', 'HEAD']
  if (noBodyMethods.some(x => method.toUpperCase() === x) && body)
    throw new Error(`Can't do a request with a body when using ${noBodyMethods.join(', ')} HTTP methods.`)

  // Check if there's a body when doing a PUT request
  if (method.toUpperCase() === 'PUT' && !body)
    throw new Error('A PUT HTTP method request should have a body.')

  const res = await fetch(`${kv}/${key}`, {
    method,
    body: body ? body : undefined,
    headers: {
      ...headers,
      'Authorization': store.state.token
    }
  })
  if (!res.ok) throw new Error(`Error ${res.status} - ${res.message}`)
  return res
}

/**
 * Lucid API wrapper
 */
export const lucidApi = {
  /**
   * Retrieve the content associated with a key
   *
   * @param {string} key Targetted key
   * @returns {Promise<Response>} Data contained in the targetted key
   * @throws {Error} Key could not be found
   * @see https://clintnetwork.gitbook.io/lucid/docs/api
   */
  getKey: key => apiCall(key),

  /**
   * Delete a key-value pair
   *
   * @param {string} key Targetted key
   * @returns {Promise<void>} The key-value pair was deleted
   * @throws {Error} Key could not be found
   * @see https://clintnetwork.gitbook.io/lucid/docs/api
   */
  deleteKey: key => apiCall(key, 'DELETE'),

  /**
   * Check a key-value pair exists
   *
   * @param {string} key Targetted key
   * @returns {Promise<void>} The key-value pair exists
   * @throws {Error} Key could not be found
   * @see https://clintnetwork.gitbook.io/lucid/docs/api
   */
  existsKey: key => apiCall(key, 'HEAD'),

  /**
   * Store any data in a key-value pair
   *
   * @param {string} key Targetted key
   * @param {any} data Any data to store in the pair
   * @returns {Promise<void>} The key-value pair was updated
   * @throws {Error} Key can't be updated
   * @see https://clintnetwork.gitbook.io/lucid/docs/api
   */
  storeKeyDataAny: (key, data) => apiCall(key, 'PUT', data),

  /**
   * Store JSON in a key-value pair
   *
   * @param {string} key Targetted key
   * @param {object} obj Object to store as JSON in the pair
   * @returns {Promise<void>} The key-value pair was updated
   * @throws {Error} Key can't be updated
   * @see https://clintnetwork.gitbook.io/lucid/docs/api
   */
  storeKeyDataJson: (key, obj) => apiCall(key, 'PUT', JSON.stringify(obj), { 'Content-Type': 'application/json' })
}
