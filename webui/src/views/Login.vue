<template>
  <div>
    <h1>Login</h1>
    <b-form @submit.prevent="onSubmit">
      <b-form-group label="Targetted key" label-for="key">
        <b-form-input
          id="key"
          v-model="token"
          required
          placeholder="key"
          :disabled="loading"
        />
      </b-form-group>

      <b-button type="submit" variant="primary" :disabled="loading">Log in</b-button>

      <b-alert :show="error" variant="danger" dismissible class="mt-3">{{ error }}</b-alert>
    </b-form>
  </div>
</template>

<script>
import { mapActions } from 'vuex'

export default {
  name: 'Login',
  data() {
    return {
      token: null,
      loading: false,
      error: null
    }
  },
  methods: {
    ...mapActions(['logIn']),

    async onSubmit() {
      if (!this.token) return this.error = 'You must enter a token.'

      this.loading = true
      this.error = null
      try {
        await this.logIn(this.token)
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
