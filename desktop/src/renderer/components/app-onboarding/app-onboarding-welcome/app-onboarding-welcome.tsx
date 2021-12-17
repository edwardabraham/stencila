import { Component, h } from '@stencil/core'
import { href } from '@stencil/router'
import Logo from '@stencila/brand/dist/logos/stencilaLogo.svg'
import { i18n } from '../../../../i18n'
import { OnboardingRouter } from '../onboardingRouter'

@Component({
  tag: 'app-onboarding-welcome',
  styleUrl: 'app-onboarding-welcome.css',
  scoped: false,
})
export class AppOnboardingWelcome {
  render() {
    return (
      <div class="app-onboarding">
        <img src={Logo} class="logo" alt="Stencila logo" />

        <h1>{i18n.t('onboarding.welcome.title')}</h1>

        <p>{i18n.t('onboarding.welcome.explanation')}</p>

        <stencila-button {...href('/onboarding/reporting', OnboardingRouter)}>
          {i18n.t('onboarding.welcome.next')}
        </stencila-button>
      </div>
    )
  }
}
