require "bundler/gem_tasks"
require "rake/testtask"

desc 'Build the Rust extension'
task :build_lib do
  sh 'cargo build --release --manifest-path crates/wasmer/Cargo.toml'
end

desc 'Install the bundle'
task :bundle_install do
  sh 'bundle config set --local path "vendor/bundle"'
  sh 'bundle install'
  sh 'cd vendor/bundle && ln -f -s ruby/*/gems/rutie-*/ rutie'
end

Rake::TestTask.new(test: [:bundle_install, :build_lib]) do |t|
  t.libs << "tests"
  t.libs << "lib"
  t.test_files = FileList["tests/*_test.rb"]
end

task :default => :test
