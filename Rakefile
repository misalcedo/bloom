task :compile do
    `cargo build`

    Dir.chdir(File.expand_path('../target/debug', __FILE__)) do
        lib_dir = RbConfig::CONFIG['libdir']
        ld_shared = RbConfig::CONFIG['LDSHARED']
        out_flag = RbConfig::CONFIG['OUTFLAG']
        ld_flags = RbConfig::CONFIG['LDFLAGS']
        dld_flags = RbConfig::CONFIG['DLDFLAGS']
        args_shared = RbConfig::CONFIG['LIBRUBYARG_SHARED']
        dl_ext = RbConfig::CONFIG['DLEXT']

        command = "#{ld_shared} #{out_flag}bloom.#{dl_ext} libbloom.dylib #{ld_flags}#{dld_flags} #{args_shared}"

        print "Executing command: #{command}"

        `#{command}`
    end


end

task :console do
    require "./target/debug/bloom"
    require "irb"

    IRB.start(__FILE__)
end

task :default => :compile
