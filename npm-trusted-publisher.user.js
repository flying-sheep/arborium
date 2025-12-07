// ==UserScript==
// @name         NPM Trusted Publisher Auto-fill for @arborium
// @namespace    http://tampermonkey.net/
// @version      1.0
// @description  Auto-fill trusted publisher settings for @arborium packages
// @author       You
// @match        https://www.npmjs.com/package/@arborium/*/access
// @grant        none
// ==/UserScript==

(function() {
    'use strict';

    // Configuration - edit these values
    const CONFIG = {
        repositoryOwner: 'bearcove',
        repositoryName: 'arborium',
        workflowFilename: 'ci.yml',
        environmentName: '', // leave empty
        autoSubmit: false // set to true to auto-submit the form
    };

    function waitForElement(selector, timeout = 5000) {
        return new Promise((resolve, reject) => {
            const element = document.querySelector(selector);
            if (element) {
                resolve(element);
                return;
            }

            const observer = new MutationObserver((mutations, obs) => {
                const el = document.querySelector(selector);
                if (el) {
                    obs.disconnect();
                    resolve(el);
                }
            });

            observer.observe(document.body, { childList: true, subtree: true });

            setTimeout(() => {
                observer.disconnect();
                reject(new Error(`Element ${selector} not found within ${timeout}ms`));
            }, timeout);
        });
    }

    function setInputValue(input, value) {
        // React inputs need special handling
        const nativeInputValueSetter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, 'value').set;
        nativeInputValueSetter.call(input, value);
        input.dispatchEvent(new Event('input', { bubbles: true }));
        input.dispatchEvent(new Event('change', { bubbles: true }));
    }

    async function fillForm() {
        console.log('[NPM Trusted Publisher] Starting auto-fill...');

        try {
            // Wait for form to load
            await waitForElement('#oidc_repositoryOwner');

            // Fill in the fields
            const ownerInput = document.querySelector('#oidc_repositoryOwner');
            const repoInput = document.querySelector('#oidc_repositoryName');
            const workflowInput = document.querySelector('#oidc_workflowName');
            const envInput = document.querySelector('#oidc_githubEnvironmentName');

            if (ownerInput) setInputValue(ownerInput, CONFIG.repositoryOwner);
            if (repoInput) setInputValue(repoInput, CONFIG.repositoryName);
            if (workflowInput) setInputValue(workflowInput, CONFIG.workflowFilename);
            if (envInput) setInputValue(envInput, CONFIG.environmentName);

            console.log('[NPM Trusted Publisher] Form filled!');

            // Find and highlight the submit button
            const submitBtn = document.querySelector('button[aria-label="Set up new trusted publisher connection"]');
            if (submitBtn) {
                submitBtn.style.boxShadow = '0 0 10px 3px #00ff00';

                if (CONFIG.autoSubmit) {
                    console.log('[NPM Trusted Publisher] Auto-submitting...');
                    submitBtn.click();
                }
            }

        } catch (err) {
            console.error('[NPM Trusted Publisher] Error:', err);
        }
    }

    // Run after page loads
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', fillForm);
    } else {
        // Small delay to let React render
        setTimeout(fillForm, 1000);
    }
})();
