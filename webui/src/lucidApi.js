import { LucidAPI } from 'lucid-ts-sdk'

/** @type {LucidAPI} Lucid API wrapper */
export let Lucid

export const initLucidWrapper = async (lucidApiEndpoint, token) => {
  Lucid = new LucidAPI(lucidApiEndpoint, token)
  return Lucid.init()
}
