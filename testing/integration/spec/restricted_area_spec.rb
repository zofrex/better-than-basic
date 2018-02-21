describe "the private area", type: :feature do
  it "redirects a logged-out client to the login page" do
    visit "/private"
    expect(page).to have_content 'You need to login to access this page'
  end
end
