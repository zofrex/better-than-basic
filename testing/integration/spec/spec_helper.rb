require 'capybara/rspec'
require 'capybara/mechanize'
require 'rack'

RSpec.configure do |config|
  config.expect_with :rspec do |expectations|
    # This option will default to `true` in RSpec 4.
    expectations.include_chain_clauses_in_custom_matcher_descriptions = true
  end

  config.mock_with :rspec do |mocks|
    # This will default to `true` in RSpec 4.
    mocks.verify_partial_doubles = true
  end

  # This option will default to `:apply_to_host_groups` in RSpec 4
  config.shared_context_metadata_behavior = :apply_to_host_groups
end

Capybara.default_driver = :mechanize
Capybara.app_host = "http://nginx:8080"
Capybara.app = Rack::Builder.new # Needed to make capybara-mechanize happy
