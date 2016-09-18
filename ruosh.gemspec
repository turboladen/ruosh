require_relative 'lib/ruosh/version'

Gem::Specification.new do |s|
  s.name = 'ruosh'
  s.version = Ruosh::VERSION
  s.author = 'Steve Loveless'
  s.homepage = 'http://github.com/turboladen/rosh'
  s.email = 'steve.loveless@gmail.com'
  s.summary = 'Ruby Object Shell'
  s.description = 'An API and shell that returns Ruby objects.'

  s.required_rubygems_version = '>=1.8.0'
  s.files = Dir.glob('{lib,spec}/**/*') + Dir.glob('*.md') +
    %w[Gemfile rosh.gemspec Rakefile]
  s.test_files = Dir.glob('{spec}/**/*')
  s.require_paths = %w[lib]

  s.add_development_dependency 'byebug'
  s.add_development_dependency 'pry'
  s.add_development_dependency 'rspec', '~> 3.3'
  s.add_development_dependency 'simplecov'
  s.add_development_dependency 'yard', '>= 0.7.2'
end
