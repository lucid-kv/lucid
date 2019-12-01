<template>
  <div class="vh-100 d-flex justify-content-center align-items-center">
    <div class="card text-center container container-sm">
      <div id="content-body">
        <div class="p-5">
          <h5>Welcome to Lucid</h5>
          <b-form @submit.prevent="onSubmit">
            <b-form-group label="Lucid API endpoint" label-for="endpoint">
              <b-form-input v-model="endpoint" id="endpoint" class="form-control text-center" placeholder="Endpoint" :disabled="loading" />
            </b-form-group>
            <b-form-group label="Lucid API token" label-for="token">
              <b-form-input v-model="token" id="token" type="password" class="form-control text-center" placeholder="Token" :disabled="loading" />
            </b-form-group>

            <b-form-checkbox v-model="rememberEndpoint">
              Remember the Lucid API endpoint
            </b-form-checkbox>

            <b-button class="my-2" type="submit" variant="primary" :disabled="loading || !token || !endpoint">Log in</b-button>

            <Loader v-if="loading" center />

            <b-alert :show="error" variant="danger" dismissible class="mt-3">{{ error }}</b-alert>
          </b-form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapActions, mapGetters } from 'vuex'

import Loader from '@/components/Loader'

export default {
  name: 'Login',
  components: {
    Loader
  },
  data() {
    return {
      endpoint: 'http://localhost:7020/api',
      rememberEndpoint: true,
      token: null,
      loading: false,
      error: null
    }
  },
  computed: {
    ...mapGetters(['LUCID_API_ENDPOINT'])
  },
  mounted() {
    // Show error notification passed by uri
    if (this.$route.query.error) this.error = this.$route.query.error

    // Load remembered Lucid API endpoint in the form
    if (this.LUCID_API_ENDPOINT) this.endpoint = this.LUCID_API_ENDPOINT
  },
  methods: {
    ...mapActions(['logIn']),

    async onSubmit() {
      if (!this.endpoint || !this.token) return this.error = 'All fields are required.'
      this.loading = true
      this.error = null
      try {
        await this.logIn({
          endpoint: this.endpoint,
          rememberEndpoint: this.rememberEndpoint,
          token: this.token
        })
      }
      catch (error) {
        console.error(error)
        this.error = error.message
      }
      finally {
        this.loading = false
      }
    }
  }
}
</script>
