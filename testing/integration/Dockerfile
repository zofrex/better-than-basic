FROM ruby:2.5.0

WORKDIR /ruby
COPY Gemfile* ./
RUN bundle install

COPY . .

CMD bundle exec rspec spec
