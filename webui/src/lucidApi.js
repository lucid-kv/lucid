import store from './store'

/** Lucid server URI */
export const LUCID_SERVER_URI = process.env.VUE_APP_LUCID_SERVER_URI || 'http://localhost:7090'

/**
 * Check a Lucid endpoint is valid
 *
 * @param {string} endpoint Lucid endpoint to check
 * @returns {Promise<void>} Valid endpoint
 * @throws {Error} Invalid endpoint
 */
export const checkLucidEndpoint = endpoint => fetch(`${endpoint}/ui/version`)
  .then(async res => {
    const err = new Error('Error - Endpoint could not be determined to be a Lucid endpoint.')
    if (!res.ok) throw err
    const version = await res.text()
    if (!version.startsWith('Lucid Version')) throw err
    return version
  })
  .catch(() => {
    throw new Error('Error - The endpoint did not answer the request.')
  })

/**
 * Check a Lucid JWT is valid
 *
 * @param {string} token Lucid JWT to check
 * @returns {Promise<void>} Valid token
 * @throws {Error} Invalid token
 */
export const checkLucidToken = token => fetch(`${store.getters.LUCID_KV_ENDPOINT}/check-token`,
  { headers: { Authorization: `Bearer ${token}` } })
  .then(async res => {
    if (!res.ok) throw new Error(`Error ${res.status} - ${(await res.json()).message}`)
    return res
  })

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
export const lucidApiCall = async (key, method = 'GET', body, headers = {}) => {
  // Check logged in
  if (!store.getters.isLoggedIn) throw new Error('You must be logged in to request a key-value pair.')

  // Check if trying to do a request with a body when not allowed to
  const noBodyMethods = ['GET', 'DELETE', 'HEAD']
  if (noBodyMethods.some(x => method.toUpperCase() === x) && body)
    throw new Error(`Can't do a request with a body when using ${noBodyMethods.join(', ')} HTTP methods.`)

  // Check if there's a body when doing a PUT request
  if (method.toUpperCase() === 'PUT' && !body)
    throw new Error('A PUT HTTP method request should have a body.')

  const res = await fetch(`${store.getters.LUCID_KV_ENDPOINT}/${key}`, {
    method,
    body: body ? body : undefined,
    headers: {
      ...headers,
      Authorization: `Bearer ${store.state.token}`
    }
  })
  if (!res.ok) {
    const error = await res.json()
    throw new Error(`Error ${res.status} - ${error.message}`)
  }
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
  getKey: key => lucidApiCall(key),

  /**
   * Delete a key-value pair
   *
   * @param {string} key Targetted key
   * @returns {Promise<void>} The key-value pair was deleted
   * @throws {Error} Key could not be found
   * @see https://clintnetwork.gitbook.io/lucid/docs/api
   */
  deleteKey: key => lucidApiCall(key, 'DELETE'),

  /**
   * Check a key-value pair exists
   *
   * @param {string} key Targetted key
   * @returns {Promise<void>} The key-value pair exists
   * @throws {Error} Key could not be found
   * @see https://clintnetwork.gitbook.io/lucid/docs/api
   */
  existsKey: key => lucidApiCall(key, 'HEAD'),

  /**
   * Store any data in a key-value pair
   *
   * @param {string} key Targetted key
   * @param {any} data Any data to store in the pair
   * @returns {Promise<void>} The key-value pair was updated
   * @throws {Error} Key can't be updated
   * @see https://clintnetwork.gitbook.io/lucid/docs/api
   */
  storeKeyDataAny: (key, data) => lucidApiCall(key, 'PUT', data),

  /**
   * Store JSON in a key-value pair
   *
   * @param {string} key Targetted key
   * @param {object} obj Object to store as JSON in the pair
   * @returns {Promise<void>} The key-value pair was updated
   * @throws {Error} Key can't be updated
   * @see https://clintnetwork.gitbook.io/lucid/docs/api
   */
  storeKeyDataJson: (key, obj) => lucidApiCall(key, 'PUT', JSON.stringify(obj), { 'Content-Type': 'application/json' })
}
