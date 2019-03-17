FROM ruby:2.6.0

RUN gem install bundler

WORKDIR /ruby
COPY Gemfile* ./
RUN bundle install

COPY . .

CMD bundle exec rspec spec
